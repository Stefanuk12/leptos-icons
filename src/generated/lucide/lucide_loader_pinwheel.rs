use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12c0-2.8 2.2-5 5-5s5 2.2 5 5 2.2 5 5 5 5-2.2 5-5" ></ path > < path d = "M7 20.7a1 1 0 1 1 5-8.7 1 1 0 1 0 5-8.6" ></ path > < path d = "M7 3.3a1 1 0 1 1 5 8.6 1 1 0 1 0 5 8.6" ></ path > < circle cy = "12" cx = "12" r = "10" ></ circle > < / > } } pub const LUCIDE_LOADER_PINWHEEL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;