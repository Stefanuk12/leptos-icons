use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m5 8 6 6" ></ path > < path d = "m4 14 6-6 2-3" ></ path > < path d = "M2 5h12" ></ path > < path d = "M7 2h1" ></ path > < path d = "m22 22-5-10-5 10" ></ path > < path d = "M14 18h6" ></ path > < / > } } pub const LUCIDE_LANGUAGES : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24")] } ;