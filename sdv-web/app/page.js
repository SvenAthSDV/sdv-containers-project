import Image from "next/image";
import styles from "./page.module.css";
import logo from './logo.png';

const API_URL = process.env.API_URL;

export default async function Home() {
  const res = await fetch(`${API_URL}/jokes/random`, { cache: 'no-store' });
  const joke = await res.json();

  return (
    <div className={styles.page}>

      <main className={styles.main}>

        <Image
          className={styles.logo}
          src={logo}
          alt="Sup de Vinci logo"
          priority
        />

        <h1>Joke of the day</h1>

        {
          joke.setup && joke.punchline && (
            <>
              <h2>{joke.setup}</h2>
              <p>{joke.punchline}</p>
            </>
          )
        }

      </main>

    </div>
  );
}