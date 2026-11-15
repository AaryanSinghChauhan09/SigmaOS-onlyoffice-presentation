//! SigmaOS-onlyoffice-presentation: ONLYOFFICE presentation editor model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for ONLYOFFICE presentation editor model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OoPresentation {
    pub id: String,
    pub name: String,
    pub slide_count: usize,
    pub format: String,
    pub enabled: bool,
}

/// Slide domain entity for ONLYOFFICE presentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OoSlide {
    pub id: String,
    pub title: String,
    pub layout: String,
}

/// Errors for the ONLYOFFICE presentation editor model domain.
#[derive(Debug, PartialEq, Eq)]
pub enum OnlyofficePresentationError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for OnlyofficePresentationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for OnlyofficePresentationError {}

/// Registry and manager for ONLYOFFICE presentation editor model.
pub struct OnlyofficePresentationManager {
    pub name: String,
    items: BTreeMap<String, OoPresentation>,
    slides: BTreeMap<String, OoSlide>,
}

impl OnlyofficePresentationManager {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            slides: BTreeMap::new(),
        }
    }

    pub fn add_item(&mut self, item: OoPresentation) -> Result<(), OnlyofficePresentationError> {
        if item.name.is_empty() {
            return Err(OnlyofficePresentationError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(OnlyofficePresentationError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    pub fn get_item(&self, id: &str) -> Option<&OoPresentation> {
        self.items.get(id)
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn add_slide(&mut self, slide: OoSlide) {
        self.slides.insert(slide.id.clone(), slide);
    }

    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    pub fn set_enabled(
        &mut self,
        id: &str,
        enabled: bool,
    ) -> Result<(), OnlyofficePresentationError> {
        self.items
            .get_mut(id)
            .map(|i| i.enabled = enabled)
            .ok_or_else(|| OnlyofficePresentationError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(id: &str, name: &str) -> OoPresentation {
        OoPresentation {
            id: id.to_string(),
            name: name.to_string(),
            slide_count: 5,
            format: "pptx".to_string(),
            enabled: true,
        }
    }

    fn make_slide(id: &str) -> OoSlide {
        OoSlide {
            id: id.to_string(),
            title: format!("Slide {id}"),
            layout: "title_and_content".to_string(),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = OnlyofficePresentationManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = OnlyofficePresentationManager::new("test");
        assert!(mgr.add_item(make_item("p1", "First Presentation")).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut mgr = OnlyofficePresentationManager::new("test");
        assert_eq!(
            mgr.add_item(make_item("p1", "")),
            Err(OnlyofficePresentationError::EmptyName)
        );
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = OnlyofficePresentationManager::new("test");
        mgr.add_item(make_item("p1", "First")).unwrap();
        assert_eq!(
            mgr.add_item(make_item("p1", "Dup")),
            Err(OnlyofficePresentationError::Duplicate("p1".to_string()))
        );
    }

    #[test]
    fn test_slide_addition_aaa() {
        let mut mgr = OnlyofficePresentationManager::new("test");
        mgr.add_slide(make_slide("s1"));
        assert_eq!(mgr.slide_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = OnlyofficePresentationManager::new("test");
        mgr.add_item(make_item("p1", "First")).unwrap();
        assert!(mgr.get_item("p1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = OnlyofficePresentationManager::new("test");
        mgr.add_item(make_item("p1", "First")).unwrap();
        assert!(mgr.set_enabled("p1", false).is_ok());
        assert!(!mgr.get_item("p1").unwrap().enabled);
        assert_eq!(
            mgr.set_enabled("none", true),
            Err(OnlyofficePresentationError::NotFound("none".to_string()))
        );
    }
}
