use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" clip - rule = "evenodd" d = "M10 1a4.5 4.5 0 0 0-4.5 4.5V9H5a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-6a2 2 0 0 0-2-2h-.5V5.5A4.5 4.5 0 0 0 10 1Zm3 8V5.5a3 3 0 1 0-6 0V9h6Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_LOCK_CLOSED : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("viewBox" , "0 0 20 20") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor")] } ;