use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 12H3" ></ path > < path d = "M16 18H3" ></ path > < path d = "M16 6H3" ></ path > < path d = "M21 12h.01" ></ path > < path d = "M21 18h.01" ></ path > < path d = "M21 6h.01" ></ path > < / > } } pub const LUCIDE_TABLE_OF_CONTENTS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;