use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse rx = "3" ry = "2" cy = "11" cx = "12" ></ ellipse > < ellipse ry = "8.5" cx = "12" rx = "10" cy = "12.5" ></ ellipse > < / > } } pub const LUCIDE_TORUS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;