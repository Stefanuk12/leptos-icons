use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12h6" ></ path > < path d = "M22 12h-6" ></ path > < path d = "M12 2v2" ></ path > < path d = "M12 8v2" ></ path > < path d = "M12 14v2" ></ path > < path d = "M12 20v2" ></ path > < path d = "m19 9-3 3 3 3" ></ path > < path d = "m5 15 3-3-3-3" ></ path > < / > } } pub const LUCIDE_FOLD_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;