use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8" ></ path > < path d = "M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" ></ path > < path d = "M12 4v6" ></ path > < path d = "M2 18h20" ></ path > < / > } } pub const LUCIDE_BED_DOUBLE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;