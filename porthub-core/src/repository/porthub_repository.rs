use crate::models::{Bookmark, Tag, Group, Workspace};
use crate::repository::error::RepositoryResult;

pub trait PortHubRepository {
    fn add_workspace(&self, workspace: &Workspace)                      -> RepositoryResult<()>;
    fn get_workspaces(&self)                                            -> RepositoryResult<Vec<Workspace>>;
    fn get_workspace_by_id(&self, id: &str)                             -> RepositoryResult<Workspace>;
    fn update_workspace(&self, workspace: &Workspace)                   -> RepositoryResult<()>;
    fn delete_workspace(&self, workspace_id: &str)                      -> RepositoryResult<()>;

    fn add_group(&self, workspace_id: &str, group: &Group)              -> RepositoryResult<()>;
    fn get_groups_by_workspace(&self, workspace_id: &str)               -> RepositoryResult<Vec<Group>>;
    fn get_group_by_id(&self, id: &str)                                 -> RepositoryResult<Group>;
    fn update_group(&self, workspace_id: &str, group: &Group)           -> RepositoryResult<()>;
    fn delete_group(&self, id: &str)                                    -> RepositoryResult<()>;

    fn add_bookmark(&self, group_id: &str, bookmark: &Bookmark)         -> RepositoryResult<()>;
    fn get_bookmarks_by_group(&self, group_id: &str)                    -> RepositoryResult<Vec<Bookmark>>;
    fn get_bookmark_by_id(&self, id: &str)                              -> RepositoryResult<Bookmark>;
    fn update_bookmark(&self, bookmark: &Bookmark)                      -> RepositoryResult<()>;
    fn delete_bookmark(&self, id: &str)                                 -> RepositoryResult<()>;

    fn add_tag(&self, workspace_id: &str, name: &str, color: &str)      -> RepositoryResult<Tag>;
    fn get_tags_by_workspace(&self, workspace_id: &str)                 -> RepositoryResult<Vec<Tag>>;
    fn get_tags_by_group(&self, group_id: &str)                         -> RepositoryResult<Vec<Tag>>;
    fn add_tag_to_bookmark(&self, bookmark_id: &str, tag_id: &str)      -> RepositoryResult<()>;
    fn remove_tag_from_bookmark(&self, bookmark_id: &str, tag_id: &str) -> RepositoryResult<()>;

    /// Search by text, name, description, url
    fn find_bookmarks(&self, query: &str)                               -> RepositoryResult<Vec<Bookmark>>;
    fn get_favorite_bookmarks(&self)                                    -> RepositoryResult<Vec<Bookmark>>;
}
