use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 14v2" ></ path > < path d = "M14 20v2" ></ path > < path d = "M14 2v2" ></ path > < path d = "M14 8v2" ></ path > < path d = "M2 15h8" ></ path > < path d = "M2 3h6a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H2" ></ path > < path d = "M2 9h8" ></ path > < path d = "M22 15h-4" ></ path > < path d = "M22 3h-2a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2" ></ path > < path d = "M22 9h-4" ></ path > < path d = "M5 3v18" ></ path > < / > } } pub const LUCIDE_TABLE_COLUMNS_SPLIT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;