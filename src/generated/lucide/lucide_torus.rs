use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse ry = "2" cx = "12" cy = "11" rx = "3" ></ ellipse > < ellipse cx = "12" ry = "8.5" rx = "10" cy = "12.5" ></ ellipse > < / > } } pub const LUCIDE_TORUS : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2")] } ;