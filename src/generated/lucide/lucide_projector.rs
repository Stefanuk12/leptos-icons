use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 7 3 5" ></ path > < path d = "M9 6V3" ></ path > < path d = "m13 7 2-2" ></ path > < circle cy = "13" cx = "9" r = "3" ></ circle > < path d = "M11.83 12H20a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h2.17" ></ path > < path d = "M16 16h2" ></ path > < / > } } pub const LUCIDE_PROJECTOR : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("height" , "24")] } ;