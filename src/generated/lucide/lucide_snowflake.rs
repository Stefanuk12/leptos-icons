use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "22" x1 = "2" y2 = "12" y1 = "12" ></ line > < line y2 = "22" x2 = "12" x1 = "12" y1 = "2" ></ line > < path d = "m20 16-4-4 4-4" ></ path > < path d = "m4 8 4 4-4 4" ></ path > < path d = "m16 4-4 4-4-4" ></ path > < path d = "m8 20 4-4 4 4" ></ path > < / > } } pub const LUCIDE_SNOWFLAKE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("fill" , "none")] } ;