use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "8" cy = "10" ></ circle > < circle cy = "10" r = "3" cx = "12" ></ circle > < path d = "M7 22h10" ></ path > < path d = "M12 22v-4" ></ path > < / > } } pub const LUCIDE_WEBCAM : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;