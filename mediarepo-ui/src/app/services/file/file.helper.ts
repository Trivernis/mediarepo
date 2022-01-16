import {downloadDir} from "@tauri-apps/api/path";
import {dialog} from "@tauri-apps/api";
import {File} from "../../../api/models/File";

export class FileHelper {

    /**
     * Opens a dialog to get a download location for the given file
     * @param {File} file
     */
    public static async getFileDownloadLocation(file: File): Promise<string | undefined> {
        let extension = FileHelper.getExtensionForMime(file.mimeType);

        const downloadDirectory = await downloadDir();
        const suggestionPath = downloadDirectory + file.cd + "." + extension;

        return await dialog.save({
            defaultPath: suggestionPath,
            filters: [{
                name: file.mimeType,
                extensions: [extension ?? "*"]
            }, {name: "All", extensions: ["*"]}]
        });
    }

    /**
     * Parses a mime into its two components
     * @param {string | undefined} mimeType
     * @returns {[string, string] | undefined}
     */
    public static parseMime(mimeType: string | undefined): [string, string] | undefined {
        if (!mimeType) {
            return undefined;
        }
        let mimeParts = mimeType.split("/");
        if (mimeParts.length < 2) {
            return undefined;
        }
        const type = mimeParts[0];
        const subtype = mimeParts[1];

        return [type, subtype];
    }

    /**
     * Returns the extension for a mime type
     * @param {string} mime
     * @returns {string | undefined}
     * @private
     */
    public static getExtensionForMime(mime: string): string | undefined {
        let parts = mime.split("/");

        if (parts.length === 2) {
            const type = parts[0];
            const subtype = parts[1];
            return FileHelper.convertMimeSubtypeToExtension(subtype);
        }
        return undefined;
    }

    private static convertMimeSubtypeToExtension(subtype: string): string {
        return subtype;
    }
}
