use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 2v14a2 2 0 0 0 2 2h14" ></ path > < path d = "M18 22V8a2 2 0 0 0-2-2H2" ></ path > < / > } } pub const LUCIDE_CROP : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;