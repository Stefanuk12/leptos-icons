use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16Zm-.75-4.75a.75.75 0 0 0 1.5 0V8.66l1.95 2.1a.75.75 0 1 0 1.1-1.02l-3.25-3.5a.75.75 0 0 0-1.1 0L6.2 9.74a.75.75 0 1 0 1.1 1.02l1.95-2.1v4.59Z" fill - rule = "evenodd" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_ARROW_UP_CIRCLE : Path = Path { path : icon_path , props : & [("fill" , "currentColor") , ("viewBox" , "0 0 20 20") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;