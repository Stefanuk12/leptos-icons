use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 1a4.5 4.5 0 0 0-4.5 4.5V9H5a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-6a2 2 0 0 0-2-2h-.5V5.5A4.5 4.5 0 0 0 10 1Zm3 8V5.5a3 3 0 1 0-6 0V9h6Z" fill - rule = "evenodd" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_LOCK_CLOSED : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 20 20") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true")] } ;