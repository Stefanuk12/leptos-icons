use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < path d = "M19.13 5.09C15.22 9.14 10 10.44 2.25 10.94" ></ path > < path d = "M21.75 12.84c-6.62-1.41-12.14 1-16.38 6.32" ></ path > < path d = "M8.56 2.75c4.37 6 6 9.42 8 17.72" ></ path > < / > } } pub const LUCIDE_DRIBBBLE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;