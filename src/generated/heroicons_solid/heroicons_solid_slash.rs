use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.256 3.042a.75.75 0 0 1 .449.962l-6 16.5a.75.75 0 1 1-1.41-.513l6-16.5a.75.75 0 0 1 .961-.449Z" clip - rule = "evenodd" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_SLASH : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("fill" , "currentColor") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;