use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M0 96C0 60.7 28.7 32 64 32l96 0c88.4 0 160 71.6 160 160s-71.6 160-160 160l-96 0 0 96c0 17.7-14.3 32-32 32s-32-14.3-32-32L0 320 0 96zM64 288l96 0c53 0 96-43 96-96s-43-96-96-96L64 96l0 192z" ></ path > < / > } } pub const FONT_AWESOME_SOLID_P : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 320 512")] } ;