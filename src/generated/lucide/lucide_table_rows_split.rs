use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 10h2" ></ path > < path d = "M15 22v-8" ></ path > < path d = "M15 2v4" ></ path > < path d = "M2 10h2" ></ path > < path d = "M20 10h2" ></ path > < path d = "M3 19h18" ></ path > < path d = "M3 22v-6a2 2 135 0 1 2-2h14a2 2 45 0 1 2 2v6" ></ path > < path d = "M3 2v2a2 2 45 0 0 2 2h14a2 2 135 0 0 2-2V2" ></ path > < path d = "M8 10h2" ></ path > < path d = "M9 22v-8" ></ path > < path d = "M9 2v4" ></ path > < / > } } pub const LUCIDE_TABLE_ROWS_SPLIT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;