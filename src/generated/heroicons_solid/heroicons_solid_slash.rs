use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path clip - rule = "evenodd" d = "M15.256 3.042a.75.75 0 0 1 .449.962l-6 16.5a.75.75 0 1 1-1.41-.513l6-16.5a.75.75 0 0 1 .961-.449Z" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_SLASH : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24")] } ;