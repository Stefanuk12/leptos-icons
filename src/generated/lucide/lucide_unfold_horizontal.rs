use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 12h6" ></ path > < path d = "M8 12H2" ></ path > < path d = "M12 2v2" ></ path > < path d = "M12 8v2" ></ path > < path d = "M12 14v2" ></ path > < path d = "M12 20v2" ></ path > < path d = "m19 15 3-3-3-3" ></ path > < path d = "m5 9-3 3 3 3" ></ path > < / > } } pub const LUCIDE_UNFOLD_HORIZONTAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;