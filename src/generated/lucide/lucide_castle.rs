use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 20v-9H2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z" ></ path > < path d = "M18 11V4H6v7" ></ path > < path d = "M15 22v-4a3 3 0 0 0-3-3a3 3 0 0 0-3 3v4" ></ path > < path d = "M22 11V9" ></ path > < path d = "M2 11V9" ></ path > < path d = "M6 4V2" ></ path > < path d = "M18 4V2" ></ path > < path d = "M10 4V2" ></ path > < path d = "M14 4V2" ></ path > < / > } } pub const LUCIDE_CASTLE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;