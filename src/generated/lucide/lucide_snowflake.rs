use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" y2 = "12" x1 = "2" x2 = "22" ></ line > < line x2 = "12" x1 = "12" y2 = "22" y1 = "2" ></ line > < path d = "m20 16-4-4 4-4" ></ path > < path d = "m4 8 4 4-4 4" ></ path > < path d = "m16 4-4 4-4-4" ></ path > < path d = "m8 20 4-4 4 4" ></ path > < / > } } pub const LUCIDE_SNOWFLAKE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;