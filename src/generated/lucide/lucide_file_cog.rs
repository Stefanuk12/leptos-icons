use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle cx = "6" cy = "14" r = "3" ></ circle > < path d = "M6 10v1" ></ path > < path d = "M6 17v1" ></ path > < path d = "M10 14H9" ></ path > < path d = "M3 14H2" ></ path > < path d = "m9 11-.88.88" ></ path > < path d = "M3.88 16.12 3 17" ></ path > < path d = "m9 17-.88-.88" ></ path > < path d = "M3.88 11.88 3 11" ></ path > < / > } } pub const LUCIDE_FILE_COG : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;