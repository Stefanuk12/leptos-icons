use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" x = "2" width = "20" rx = "2" y = "4" ></ rect > < path d = "M2 14h20" ></ path > < path d = "M12 20v-6" ></ path > < / > } } pub const LUCIDE_TOUCHPAD : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2")] } ;