use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" x = "2" height = "16" y = "4" rx = "2" ></ rect > < path d = "M2 14h20" ></ path > < path d = "M12 20v-6" ></ path > < / > } } pub const LUCIDE_TOUCHPAD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2")] } ;