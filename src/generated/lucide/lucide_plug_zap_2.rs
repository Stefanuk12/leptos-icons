use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m13 2-2 2.5h3L12 7" ></ path > < path d = "M10 14v-3" ></ path > < path d = "M14 14v-3" ></ path > < path d = "M11 19c-1.7 0-3-1.3-3-3v-2h8v2c0 1.7-1.3 3-3 3Z" ></ path > < path d = "M12 22v-3" ></ path > < / > } } pub const LUCIDE_PLUG_ZAP_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;