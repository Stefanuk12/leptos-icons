use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 4v16" ></ path > < path d = "M9 4v16" ></ path > < path d = "M14 4v16" ></ path > < path d = "M19 4v16" ></ path > < path d = "M22 6 2 18" ></ path > < / > } } pub const LUCIDE_TALLY_5 : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;