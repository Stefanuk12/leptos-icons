use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m8 6 4-4 4 4" ></ path > < path d = "M12 2v10.3a4 4 0 0 1-1.172 2.872L4 22" ></ path > < path d = "m20 22-5-5" ></ path > < / > } } pub const LUCIDE_MERGE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;