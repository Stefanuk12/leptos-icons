use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse ry = "2" cx = "12" rx = "3" cy = "11" ></ ellipse > < ellipse cy = "12.5" ry = "8.5" rx = "10" cx = "12" ></ ellipse > < / > } } pub const LUCIDE_TORUS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;