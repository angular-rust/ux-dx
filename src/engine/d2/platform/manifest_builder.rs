/// Creates the asset manifests from the files in /assets
pub struct ManifestBuilder {}

impl ManifestBuilder {
    // macro
    // static
    pub fn use_(assets_dir: String) {
        // match Context::getType("d2::asset::Manifest") {
        //     TInst(cl, _) => {
        //         let cl = cl.get();
        //         let meta = cl.meta;

        //         if !assetsDir.endsWith("/") {
        //             assetsDir += "/";
        //         }

        //         let data = {};
        //         for packName in FileSystem::readDirectory(assetsDir) {
        //             let entries = Vec::new();
        //             if FileSystem::isDirectory(assetsDir + packName) {
        //                 for file in Self::readRecursive(assetsDir + packName, "") {
        //                     let path = assetsDir + packName + "/" + file;
        //                     // entries.push({
        //                     //     name: file,
        //                     //     md5: Context.signature(File.getBytes(path)),
        //                     //     bytes: FileSystem::stat(path).size,
        //                     // });
        //                 }
        //             }
        //             Reflect::setField(data, packName, entries);
        //         }

        //         meta.remove("assets");
        //         // meta.add("assets", [macro $v{data}], cl.pos);
        //     }

        //     _ => {
        //         panic!("assert");
        //     }
        // }

        // return macro {};
        unimplemented!()
    }

    // static
    // dir = ""
    fn read_recursive(root: String, dir: String) {
        // let result = Vec::new();
        // for (file in readDirectoryNoHidden(root + "/" + dir)) {
        //     let fullPath = root + "/" + dir + "/" + file;
        //     let relPath = if (dir == "") file else dir + "/" + file;
        //     if (FileSystem::isDirectory(fullPath)) {
        //         result = result.concat(readRecursive(root, relPath));
        //     } else {
        //         result.push(relPath);
        //     }
        // }
        // return result;
        unimplemented!()
    }

    #[allow(unused_assignments)]
    // static
    fn read_directory_no_hidden(dir: String) {
        let mut dir = dir;
        if let Some(last) = dir.chars().last() {
            if last == '/' {
                // Trim off the trailing slash. On Windows, FileSystem::exists() doesn't find directories
                // with trailing slashes?
                dir = dir[..dir.len() - 1].to_string();
            }
        }

        // if FileSystem::exists(dir) && FileSystem::isDirectory(dir) {
        //     FileSystem::readDirectory(dir).filter(|file| return file.chars().nth(0).unwrap_or_default() != '.'.to_digit(10))
        // } else {
        //     Vec::new()
        // }
        unimplemented!()
    }
}
