use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 10s3-3 3-8" ></ path > < path d = "M22 10s-3-3-3-8" ></ path > < path d = "M10 2c0 4.4-3.6 8-8 8" ></ path > < path d = "M14 2c0 4.4 3.6 8 8 8" ></ path > < path d = "M2 10s2 2 2 5" ></ path > < path d = "M22 10s-2 2-2 5" ></ path > < path d = "M8 15h8" ></ path > < path d = "M2 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1" ></ path > < path d = "M14 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1" ></ path > < / > } } pub const LUCIDE_THEATER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none")] } ;