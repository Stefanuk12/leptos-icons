use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < path d = "m19 9-5 5-4-4-3 3" ></ path > < / > } } pub const LUCIDE_LINE_CHART : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none")] } ;