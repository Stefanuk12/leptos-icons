use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" ></ path > < path d = "M18.375 2.625a1 1 0 0 1 3 3l-9.013 9.014a2 2 0 0 1-.853.505l-2.873.84a.5.5 0 0 1-.62-.62l.84-2.873a2 2 0 0 1 .506-.852z" ></ path > < / > } } pub const LUCIDE_SQUARE_PEN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;