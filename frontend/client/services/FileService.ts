/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { FileData } from '../models/FileData';
import type { FilePage } from '../models/FilePage';
import type { FileStats } from '../models/FileStats';
import type { MessageResponse } from '../models/MessageResponse';
import type { UploadFile } from '../models/UploadFile';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class FileService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Upload a file
     * Upload a file
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     *
     * @param formData
     * @returns FileData
     * @throws ApiError
     */
    public upload(
        formData: UploadFile,
    ): CancelablePromise<FileData> {
        return this.httpRequest.request({
            method: 'POST',
            url: '/api/file',
            formData: formData,
            mediaType: 'multipart/form-data',
            errors: {
                409: `File already uploaded`,
                413: `File too large`,
            },
        });
    }

    /**
     * Get a paginated list of files
     * Get a paginated list of files
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     *
     * @param pageNumber Page to get files by (starts at 1)
     * @param query Query by file name similarity
     * @returns FilePage
     * @throws ApiError
     */
    public list(
        pageNumber: number,
        query?: string,
    ): CancelablePromise<FilePage> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/file/list/{page_number}',
            path: {
                'page_number': pageNumber,
            },
            query: {
                'query': query,
            },
            errors: {
                400: `Invalid page number`,
                404: `Page not found`,
            },
        });
    }

    /**
     * Get file stats for user
     * Get file stats for user
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     *
     * @returns FileStats
     * @throws ApiError
     */
    public stats(): CancelablePromise<FileStats> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/file/stats',
        });
    }

    /**
     * Get file data by ID
     * Get file data by ID
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     *
     * @param fileId File ID
     * @returns FileData
     * @throws ApiError
     */
    public info(
        fileId: string,
    ): CancelablePromise<FileData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/file/{file_id}',
            path: {
                'file_id': fileId,
            },
            errors: {
                403: `Access denied`,
                404: `File not found`,
            },
        });
    }

    /**
     * Delete file data by ID
     * Delete file data by ID
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     *
     * @param fileId File ID
     * @returns MessageResponse File deleted
     * @throws ApiError
     */
    public deleteFile(
        fileId: string,
    ): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'DELETE',
            url: '/api/file/{file_id}',
            path: {
                'file_id': fileId,
            },
            errors: {
                403: `Access denied`,
                404: `File not found`,
            },
        });
    }

}
