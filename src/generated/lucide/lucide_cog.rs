use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z" ></ path > < path d = "M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z" ></ path > < path d = "M12 2v2" ></ path > < path d = "M12 22v-2" ></ path > < path d = "m17 20.66-1-1.73" ></ path > < path d = "M11 10.27 7 3.34" ></ path > < path d = "m20.66 17-1.73-1" ></ path > < path d = "m3.34 7 1.73 1" ></ path > < path d = "M14 12h8" ></ path > < path d = "M2 12h2" ></ path > < path d = "m20.66 7-1.73 1" ></ path > < path d = "m3.34 17 1.73-1" ></ path > < path d = "m17 3.34-1 1.73" ></ path > < path d = "m11 13.73-4 6.93" ></ path > < / > } } pub const LUCIDE_COG : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;