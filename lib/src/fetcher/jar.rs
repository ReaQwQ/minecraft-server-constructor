use std::path::Path;
use std::io::{Read, Write};
use zip::ZipArchive;
use crate::error::MsbError;

/**
 * 説明: JARファイル（ZIP形式）からリソースを抽出し、加工するユーティリティ
 */
pub struct JarExtractor;

impl JarExtractor {
    /**
     * 説明: JAR内のリソースを検索し、特定の文字列置換を行ってからファイルとして書き出す
     * @param jar_path 対象となるJARファイルのパス
     * @param internal_path JAR内部でのファイルパス
     * @param output_path 抽出後の保存先パス
     * @param replacements 置換ルールのリスト (旧文字列, 新文字列)
     * @requires ファイル読み書き権限, 有効なJARファイル
     * @return 抽出・置換に成功した場合はOk(true), 見つからない場合はOk(false)
     */
    pub fn extract_and_replace(
        jar_path: &Path,
        internal_path: &str,
        output_path: &Path,
        replacements: Vec<(&str, &str)>,
    ) -> Result<bool, MsbError> {
        let file = std::fs::File::open(jar_path)?;
        let mut archive = ZipArchive::new(file).map_err(|_| MsbError::ParseError("Invalid JAR format".to_string()))?;

        let index = (0..archive.len()).find(|&i| {
            let file = archive.by_index(i).unwrap();
            file.name() == internal_path || file.name().ends_with(&format!("/{}", internal_path))
        });

        if let Some(i) = index {
            let mut internal_file = archive.by_index(i).unwrap();
            let mut content = String::new();
            if internal_file.read_to_string(&mut content).is_ok() {
                for (old, new) in replacements {
                    content = content.replace(old, new);
                }
                if let Some(p) = output_path.parent() {
                    std::fs::create_dir_all(p)?;
                }
                let mut out_file = std::fs::File::create(output_path)?;
                out_file.write_all(content.as_bytes())?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    /**
     * 説明: JAR内のリソースを検索し、その内容を文字列として取得する
     * @param jar_path 対象となるJARファイルのパス
     * @param internal_path JAR内部でのファイルパス
     * @requires ファイル読み取り権限
     * @return 成功時はファイル内容の文字列 (Option)
     */
    pub fn get_resource_as_string(jar_path: &Path, internal_path: &str) -> Result<Option<String>, MsbError> {
        let file = std::fs::File::open(jar_path)?;
        let mut archive = ZipArchive::new(file).map_err(|_| MsbError::ParseError("Invalid JAR format".to_string()))?;

        let index = (0..archive.len()).find(|&i| {
            let file = archive.by_index(i).unwrap();
            file.name() == internal_path || file.name().ends_with(&format!("/{}", internal_path))
        });

        if let Some(i) = index {
            let mut internal_file = archive.by_index(i).unwrap();
            let mut content = String::new();
            if internal_file.read_to_string(&mut content).is_ok() {
                return Ok(Some(content));
            }
        }

        Ok(None)
    }

    /**
     * 説明: JAR内のリソースを検索し、その内容を文字列として取得する（get_resource_as_stringのショートハンド）
     * @param jar_path 対象となるJARファイルのパス
     * @param internal_path JAR内部でのファイルパス
     * @return 成功時はファイル内容の文字列
     */
    pub fn extract_to_string(jar_path: &Path, internal_path: &str) -> Result<String, MsbError> {
        Self::get_resource_as_string(jar_path, internal_path)?
            .ok_or_else(|| MsbError::ParseError(format!("Resource not found: {}", internal_path)))
    }
}
