use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 12s2.545-5 7-5c4.454 0 7 5 7 5s-2.546 5-7 5c-4.455 0-7-5-7-5z" ></ path > < path d = "M12 13a1 1 0 1 0 0-2 1 1 0 0 0 0 2z" ></ path > < path d = "M21 17v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2" ></ path > < path d = "M21 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2" ></ path > < / > } } pub const LUCIDE_VIEW : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;