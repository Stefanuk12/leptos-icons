use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h3" ></ path > < path d = "M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3" ></ path > < path d = "M12 20v2" ></ path > < path d = "M12 14v2" ></ path > < path d = "M12 8v2" ></ path > < path d = "M12 2v2" ></ path > < / > } } pub const LUCIDE_FLIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;