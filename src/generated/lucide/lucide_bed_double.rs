use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8" ></ path > < path d = "M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" ></ path > < path d = "M12 4v6" ></ path > < path d = "M2 18h20" ></ path > < / > } } pub const LUCIDE_BED_DOUBLE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;