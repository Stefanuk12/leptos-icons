use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m8 6 4-4 4 4" ></ path > < path d = "M12 2v10.3a4 4 0 0 1-1.172 2.872L4 22" ></ path > < path d = "m20 22-5-5" ></ path > < / > } } pub const LUCIDE_MERGE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;