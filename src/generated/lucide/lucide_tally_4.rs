use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 4v16" ></ path > < path d = "M9 4v16" ></ path > < path d = "M14 4v16" ></ path > < path d = "M19 4v16" ></ path > < / > } } pub const LUCIDE_TALLY_4 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;