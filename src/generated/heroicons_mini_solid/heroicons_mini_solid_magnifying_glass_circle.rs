use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6.5 9a2.5 2.5 0 1 1 5 0 2.5 2.5 0 0 1-5 0Z" ></ path > < path fill - rule = "evenodd" d = "M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16ZM9 5a4 4 0 1 0 2.248 7.309l1.472 1.471a.75.75 0 1 0 1.06-1.06l-1.471-1.472A4 4 0 0 0 9 5Z" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_MAGNIFYING_GLASS_CIRCLE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 20 20") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true")] } ;