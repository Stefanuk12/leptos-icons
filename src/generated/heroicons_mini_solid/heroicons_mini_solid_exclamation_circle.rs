use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path clip - rule = "evenodd" d = "M18 10a8 8 0 1 1-16 0 8 8 0 0 1 16 0Zm-8-5a.75.75 0 0 1 .75.75v4.5a.75.75 0 0 1-1.5 0v-4.5A.75.75 0 0 1 10 5Zm0 10a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" fill - rule = "evenodd" ></ path > < / > } } pub const HeroiconsMiniSolidExclamationCircle : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 20 20") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;