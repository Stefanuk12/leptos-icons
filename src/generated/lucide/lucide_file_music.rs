use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "16" cx = "14" r = "2" ></ circle > < circle cx = "6" r = "2" cy = "18" ></ circle > < path d = "M4 12.4V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2h-7.5" ></ path > < path d = "M8 18v-7.7L16 9v7" ></ path > < / > } } pub const LUCIDE_FILE_MUSIC : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;