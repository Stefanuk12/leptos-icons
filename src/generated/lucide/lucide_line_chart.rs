use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < path d = "m19 9-5 5-4-4-3 3" ></ path > < / > } } pub const LUCIDE_LINE_CHART : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;