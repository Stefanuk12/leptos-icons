use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 15 4-8 4 8" ></ path > < path d = "M4 13h6" ></ path > < path d = "M15 11h4.5a2 2 0 0 1 0 4H15V7h4a2 2 0 0 1 0 4" ></ path > < / > } } pub const LUCIDE_CASE_UPPER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;