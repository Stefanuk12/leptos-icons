use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 12H3" ></ path > < path d = "M16 18H3" ></ path > < path d = "M16 6H3" ></ path > < path d = "M21 12h.01" ></ path > < path d = "M21 18h.01" ></ path > < path d = "M21 6h.01" ></ path > < / > } } pub const LUCIDE_TABLE_OF_CONTENTS : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;