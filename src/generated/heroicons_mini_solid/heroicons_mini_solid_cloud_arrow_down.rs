use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.5 17a4.5 4.5 0 0 1-1.44-8.765 4.5 4.5 0 0 1 8.302-3.046 3.5 3.5 0 0 1 4.504 4.272A4 4 0 0 1 15 17H5.5Zm5.25-9.25a.75.75 0 0 0-1.5 0v4.59l-1.95-2.1a.75.75 0 1 0-1.1 1.02l3.25 3.5a.75.75 0 0 0 1.1 0l3.25-3.5a.75.75 0 1 0-1.1-1.02l-1.95 2.1V7.75Z" fill - rule = "evenodd" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_CLOUD_ARROW_DOWN : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 20 20") , ("data-slot" , "icon") , ("fill" , "currentColor")] } ;