use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 7 4.41-4.41A2 2 0 0 1 7.83 2h8.34a2 2 0 0 1 1.42.59L22 7" ></ path > < path d = "M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" ></ path > < path d = "M15 22v-4a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v4" ></ path > < path d = "M2 7h20" ></ path > < path d = "M22 7v3a2 2 0 0 1-2 2v0a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 16 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 12 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 8 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 4 12v0a2 2 0 0 1-2-2V7" ></ path > < / > } } pub const LUCIDE_STORE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24")] } ;