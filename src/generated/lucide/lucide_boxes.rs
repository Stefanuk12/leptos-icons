use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2.97 12.92A2 2 0 0 0 2 14.63v3.24a2 2 0 0 0 .97 1.71l3 1.8a2 2 0 0 0 2.06 0L12 19v-5.5l-5-3-4.03 2.42Z" ></ path > < path d = "m7 16.5-4.74-2.85" ></ path > < path d = "m7 16.5 5-3" ></ path > < path d = "M7 16.5v5.17" ></ path > < path d = "M12 13.5V19l3.97 2.38a2 2 0 0 0 2.06 0l3-1.8a2 2 0 0 0 .97-1.71v-3.24a2 2 0 0 0-.97-1.71L17 10.5l-5 3Z" ></ path > < path d = "m17 16.5-5-3" ></ path > < path d = "m17 16.5 4.74-2.85" ></ path > < path d = "M17 16.5v5.17" ></ path > < path d = "M7.97 4.42A2 2 0 0 0 7 6.13v4.37l5 3 5-3V6.13a2 2 0 0 0-.97-1.71l-3-1.8a2 2 0 0 0-2.06 0l-3 1.8Z" ></ path > < path d = "M12 8 7.26 5.15" ></ path > < path d = "m12 8 4.74-2.85" ></ path > < path d = "M12 13.5V8" ></ path > < / > } } pub const LUCIDE_BOXES : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;