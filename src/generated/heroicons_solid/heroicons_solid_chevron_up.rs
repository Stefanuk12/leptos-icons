use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" clip - rule = "evenodd" d = "M11.47 7.72a.75.75 0 0 1 1.06 0l7.5 7.5a.75.75 0 1 1-1.06 1.06L12 9.31l-6.97 6.97a.75.75 0 0 1-1.06-1.06l7.5-7.5Z" ></ path > < / > } } pub const HEROICONS_SOLID_CHEVRON_UP : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("fill" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;