use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22v-6" ></ path > < path d = "M12 8V2" ></ path > < path d = "M4 12H2" ></ path > < path d = "M10 12H8" ></ path > < path d = "M16 12h-2" ></ path > < path d = "M22 12h-2" ></ path > < path d = "m15 19-3-3-3 3" ></ path > < path d = "m15 5-3 3-3-3" ></ path > < / > } } pub const LUCIDE_FOLD_VERTICAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;